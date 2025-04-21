module Himg
  class Railtie
    # Handles .himg templates by calling through to Himg.render
    class TemplateHandler
      def self.call(_template, source)
        <<-CODE
          Himg.render(#{source.inspect}).pack("C*")
        CODE
      end
    end

    # Handles .himg.erb templates by pre-processing with Erb, possibly appending
    # to @output_buffer or calling .safe_append, and then passing the result to
    # Himg.render
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
ActionView::Template.register_template_handler "himg.erb", Himg::Railtie::ErbTemplateHandler
