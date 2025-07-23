require "uri"

module Himg
  class BaseUrl
    def initialize(path_or_url)
      path_or_url = path_or_url.to_s.strip
      return if path_or_url&.empty?

      @url = URI.parse(path_or_url)
      @url.scheme = "file" unless @url.scheme

      raise Himg::Error, "Invalid base_url #{path_or_url}" if @url.path.empty?

      @url.path += "/" unless @url.path.end_with?("/")
    end

    def to_s
      @url&.to_s
    end
  end
end
