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

    initializer "himg.controller_renderer" do
      ActionController::Renderers.add :himg do |obj, options|
        png_data = Himg.render(obj)
        send_data png_data.pack("C*"), type: "image/png", disposition: "inline"
      end
    end
  end
end
