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

    if Rails.env.development?
      config.cache_classes = false
      config.eager_load = false
      config.reload_classes_only_on_change = true
      config.action_view.cache_template_loading = false
    end

    config.root = File.expand_path('..', __dir__)
  end
end
