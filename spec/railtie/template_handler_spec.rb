require "rails_helper"

RSpec.describe Himg::Railtie::TemplateHandler do
  describe ".call" do
    let(:template) { instance_double("ActionView::Template") }
    let(:source) { "<html><body>Frankie</body></html>" }

    it "returns code that calls Himg.render" do
      result = described_class.call(template, source)

      expect(result).to include("Himg.render")
      expect(result).to include("<body>Frankie</body>")
    end

    it "evaluates to png data" do
      result = described_class.call(template, source)

      expect(eval(result).pack("C*")).to start_with("\x89PNG".b)
    end
  end
end
