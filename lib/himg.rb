# frozen_string_literal: true

require_relative "himg/version"
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

  def self.render(html, width: 720, height: 405, truncate: true, verbose: false)
    render_to_string(html, "width" => width, "height" => height, "truncate" => truncate, "verbose" => verbose)
  end
end
