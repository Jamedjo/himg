module Himg
  class Railtie < Rails::Railtie
    initializer "himg.configure_rails_initialization" do
      require 'himg/railtie/template_handler'
    end
  end
end
