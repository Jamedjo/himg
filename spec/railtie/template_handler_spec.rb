require "rails_helper"

RSpec.describe "Himg::Railtie::TemplateHandler", type: :railtie do
  describe ".call" do
    subject(:described_class) { Himg::Railtie::TemplateHandler }

    let(:template) { instance_double("ActionView::Template") }
    let(:source) { "<html><body>Frankie</body></html>" }

    it "returns code that calls Himg.render" do
      result = described_class.call(template, source)

      expect(result).to include("Himg.render")
      expect(result).to include("<body>Frankie</body>")
    end

    it "evaluates to png data" do
      result = described_class.call(template, source)

      expect(eval(result)).to start_with("\x89PNG".b)
    end
  end
end

RSpec.describe "Himg::Railtie::ErbTemplateHandler", type: :railtie do
  describe ".call" do
    subject(:described_class) { Himg::Railtie::ErbTemplateHandler }

    let(:source) { "<html><body><%= 'eiknarF'.reverse %></body></html>" }
    let(:view_path) { "/virtual/app/views/users/show.html.erb" }
    let(:template) do
      ActionView::Template.new(source, view_path, described_class, locals: [], format: 'himg.erb')
    end
    let(:view_context) do
      ActionView::Base.new(ActionView::LookupContext.new([]), {}, nil)
    end

    before do
      # Needed on Rails 7.0 only
      view_context.instance_variable_set(:"@output_buffer", ActionView::OutputBuffer.new)
    end

    it "evaluates to png data" do
      result = described_class.call(template, source)

      output = view_context.instance_eval(result)

      expect(output).to start_with("\x89PNG".b)
    end

    it "pre-processes with ERB" do
      result = described_class.call(template, source)

      expect(Himg).to receive(:render).with(/<body>Frankie<\/body>/).and_return([])

      view_context.instance_eval(result)
    end
  end
end
