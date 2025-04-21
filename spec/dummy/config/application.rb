require "rails"
require "action_controller/railtie"
require "action_view/railtie"
require "action_dispatch/railtie"

require "himg"

module Dummy
  class Application < Rails::Application
    config.load_defaults Rails::VERSION::STRING.to_f
    config.cache_classes = true
    config.eager_load = false
    config.active_support.deprecation = :stderr

    config.root = File.expand_path('..', __dir__)
  end
end
