require "thor"
require "himg"
require "open-uri"
require "uri"

module Himg
  class CLI < Thor
    CLI_ONLY_OPTIONS = [:stdin, :http_headers].freeze

    default_command :usage

    desc "usage", "Show usage for screenshot command", hide: true
    def usage
      puts "himg v#{VERSION}"
      puts
      puts "Converts HTML to PNG images using a lightweight, minimal renderer."
      puts "Ideal for generating OpenGraph images from purpose-built HTML."
      puts
      CLI.command_help(Thor::Base.shell.new, 'screenshot')
    end

    desc "screenshot [SOURCE_HTML] DESTINATION_PNG [OPTIONS]", "Render HTML to a png screenshot"

    option :width, type: :numeric, desc: "Sets the width of the rendered content.", default: 720
    option :height, type: :numeric, desc: "Sets the desired height of the rendered output.", default: 405
    option :truncate, type: :boolean, desc: "Keeps the image height fixed instead of expanding to include the full page.", default: true
    option :verbose, type: :boolean, desc: "Enables detailed logging for debugging and profiling.", default: false
    option :disable_fetch, type: :boolean, desc: "Skip fetching file/http resources (stylesheets, images, fonts, etc)", default: false
    option :fetch_timeout, type: :numeric, desc: "Timeout in seconds for fetching resources", default: 10
    option :gpu, type: :boolean, desc: "Use GPU renderer instead of CPU renderer", default: false
    option :http_headers, type: :hash, desc: "HTTP headers sent when fetching the SOURCE_HTML (e.g. --http-headers \"Authorization: Bearer token\" \"Content-Type: application/json\")"
    option :base_url, desc: "Base URL used to resolve relative URLs"
    option :stdin, type: :boolean, desc: "Read HTML content from stdin instead of a file", default: false

    long_desc <<-LONGDESC
      `himg screenshot` takes a path to an HTML file and will render a png image with the output.

      It takes a SOURCE, which can be a file path, a URL to fetch, or piped from stdin.

      If SOURCE_HTML is omitted, HTML content will be read from stdin.
      You can also use the --stdin option to explicitly read from stdin.

      The DESTINATION_PNG must be a local file path.

      Examples:
        himg screenshot input.html output.png
        himg screenshot https://himg.jamedjo.co.uk output.png
        echo '<h1>Hello</h1>' | himg screenshot --stdin output.png

      CAVEATS: This uses a lightweight HTML parser instead of a full browser, so does not support all features.
      Additionally it does not use a JavaScript engine, so will screenshot the page as-is and would not work for all webpages.
    LONGDESC
    def screenshot(first_arg = nil, second_arg = nil)
      url, destination = parse_screenshot_args(first_arg, second_arg)

      options[:http_headers]&.transform_values!(&:strip)

      Document.new(url, options).load do |content|
        render_options = options.transform_keys(&:to_sym)
          .reject { |k, _| CLI_ONLY_OPTIONS.include?(k) }
        render_options[:base_url] ||= base_directory_url(url) if Document.http_url?(url)

        png = Himg.render(content, **render_options)

        File.open(destination, "wb") { |f| f.write(png) }
      end
    end

    private

    def parse_screenshot_args(first_arg, second_arg)
      raise Thor::RequiredArgumentMissingError unless first_arg

      case
      when options[:stdin]
        raise Thor::RequiredArgumentMissingError if second_arg
        [nil, first_arg]
      when second_arg
        [first_arg, second_arg]
      else
        raise Thor::RequiredArgumentMissingError if $stdin.tty?
        [nil, first_arg]
      end
    rescue Thor::RequiredArgumentMissingError
      CLI.command_help(Thor::Base.shell.new, 'screenshot')
      raise
    end

    def base_directory_url(url)
      URI.join(url, ".").to_s
    end

    def self.exit_on_failure?
      true
    end

    class Document
      def initialize(source, options)
        @source = source
        @options = options
        @http_headers = options[:http_headers]
      end

      def self.http_url?(url)
        url.to_s =~ %r{\Ahttps?\://}
      end

      def stdin?
        @source.nil?
      end

      def load(&block)
        if stdin?
          yield($stdin.read)
        elsif self.class.http_url?(@source)
          args = [@source]
          args << @http_headers if @http_headers

          URI.open(*args) do |input|
            yield(input.binmode.read)
          end
        else
          File.open(@source) do |f|
            yield(f.read)
          end
        end
      end
    end
  end
end
