module Himg
  class Railtie < Rails::Railtie
    initializer "himg.configure_rails_initialization" do
      ActiveSupport.on_load(:action_view) do
        require 'himg/railtie/template_handler'
      end
    end

    initializer "himg.mime_types" do
      Mime::Type.register "image/png", :himg
    end
  end
end
