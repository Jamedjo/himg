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
  #class Error < StandardError; end
end
