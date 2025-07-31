# frozen_string_literal: true

require_relative "himg/version"
require_relative "himg/base_url"
require "himg/railtie" if defined?(Rails::Railtie)

# Attempt to load a versioned extension based on the Ruby version.
# Fall back to loading the non-versioned extension if version-specific loading fails.
begin
  RUBY_VERSION =~ /(\d+\.\d+)/
  require "#{Regexp.last_match(1)}/himg/himg"
rescue LoadError
  require "himg/himg"
end

# The Hyper Image Generator
#
# Converts HTML to an Image for a minimal subset of HTML and CSS
module Himg
  RENDER_OPTIONS = %i[width height truncate verbose].freeze
  class Error < StandardError; end

  def self.render(html, width: 720, height: 405, truncate: true, verbose: false, base_url: nil, disable_fetch: false, fetch_timeout: 10, gpu: false)
    render_to_string(html, "width" => width.to_i, "height" => height.to_i, "truncate" => truncate, "verbose" => verbose, "base_url" => BaseUrl.new(base_url).to_s, "disable_fetch" => disable_fetch, "fetch_timeout" => fetch_timeout.to_f, "gpu" => gpu)
  end
end
