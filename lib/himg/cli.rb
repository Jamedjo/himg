require "thor"
require "himg"
require "open-uri"

module Himg
  class CLI < Thor
    desc "screenshot SOURCE_HTML DESTINATION_PNG [OPTIONS]", "Render HTML to a png screenshot"

    option :width, type: :numeric, desc: "Sets the width of the rendered content.", default: 720
    option :height, type: :numeric, desc: "Sets the desired height of the rendered output.", default: 405
    option :truncate, type: :boolean, desc: "Keeps the image height fixed instead of expanding to include the full page.", default: true
    option :verbose, type: :boolean, desc: "Enables detailed logging for debugging and profiling.", default: false
    option :disable_fetch, type: :boolean, desc: "Skip fetching file/http resources (stylesheets, images, fonts, etc)", default: false
    option :http_headers, desc: "HTTP Headers to use when fetching remote resource"
    option :base_url, desc: "Base URL used to resolve relative URLs"

    long_desc <<-LONGDESC
      `himg screenshot` takes a path to an HTML file and will render a png image with the output.

      It takes a SOURCE, which can be a file path or a URL to fetch.

      The DESTINATION_PNG must be a local file path.

      CAVEATS: This uses a lightweight HTML parser instead of a full browser, so does not support all features.
      Additionally it does not use a JavaScript engine, so will screenshot the page as-is and would not work for all webpages.
    LONGDESC
    def screenshot(url, destination)
      Document.new(url, options).load do |content|
        png = Himg.render(content, **options.transform_keys(&:to_sym))

        File.open(destination, "wb") { |f| f.write(png) }
      end
    end

    private

    def self.exit_on_failure?
      true
    end

    class Document
      def initialize(source, options)
        @source = source
        @options = options
      end

      def http_url?
        @source =~ %r{\Ahttps?\://}
      end

      def load(&block)
        if http_url?
          URI.send(:open, @source) do |input|
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
