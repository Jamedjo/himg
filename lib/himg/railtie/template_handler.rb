module Himg
  class Railtie
    class TemplateHandler
      def self.call(template, source)
        <<-CODE
          Himg.render(#{source.inspect}).pack("C*")
        CODE
      end
    end

    class ErbTemplateHandler
      def self.call(template, source)
        erb_handler = ActionView::Template.registered_template_handler(:erb)
        preprocessed_view_code = erb_handler.call(template, source)

        <<-CODE
          output = begin
            #{preprocessed_view_code}
          end
          Himg.render(output).pack("C*")
        CODE
      end
    end
  end
end

ActionView::Template.register_template_handler :himg, Himg::Railtie::TemplateHandler
ActionView::Template.register_template_handler 'himg.erb', Himg::Railtie::ErbTemplateHandler
