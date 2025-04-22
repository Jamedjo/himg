module Himg
  # Himg::Railtie
  #
  # Allows you to render Png images directly from rails using HTML based templates.
  #
  # View templates can be registered with `.himg` and will be converted from
  # HTML to an image, or with `.himg.erb` to insert dynamic content. A renderer
  # also allows direct rendering of HTML without going through a template.
  #
  # Example usage:
  # - `show.himg.erb` renders an image based on controller data.
  # - `render himg: '<div>My Data</div>'` generates an image from HTML.
  #
  # This works by registering a template handler, which will be called by Rails'
  # default_render method when a view template exists matching the current action
  # in a request where the format extension is .himg or a png is requested.
  #
  # The direct render works similarly, but instead of checking the request format
  # it looks for the :himg key passed into a Rails render function.
  class Railtie < Rails::Railtie
    initializer "himg.configure_rails_initialization" do
      ActiveSupport.on_load(:action_view) do
        require "himg/railtie/template_handler"
      end
    end

    initializer "himg.mime_types" do
      Mime::Type.register "image/png", :himg
    end

    initializer "himg.controller_renderer" do
      ActionController::Renderers.add :himg do |obj, _options|
        png_data = Himg.render(obj)
        send_data png_data, type: "image/png", disposition: "inline"
      end
    end
  end
end
