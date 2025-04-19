module Himg
  class Railtie
    class TemplateHandler
      def self.call(template, source)
        <<-CODE
          Himg.render(#{source.inspect})#.html_safe
        CODE
      end
    end
  end
end

ActionView::Template.register_template_handler :himg, Himg::Railtie::TemplateHandler
