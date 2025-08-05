# frozen_string_literal: true

require "spec_helper"

RSpec.describe Himg::Renderer do
  describe "#new" do
    it "creates a new renderer instance" do
      renderer = Himg::Renderer.new

      expect(renderer).to be_a(Himg::Renderer)
    end
  end

  describe "#render" do
    let(:renderer) { Himg::Renderer.new }

    it "renders HTML to PNG" do
      png_string = renderer.render("<html></html>", {})

      expect(png_string).to start_with("\x89PNG\r\n\x1A\n".b)
    end

    it "handles invalid parameters at the Rust boundary" do
      expect { renderer.render(123, {}) }.to raise_error(TypeError)
    end

    it "accepts an options hash" do
      png_string = renderer.render("<html></html>", {"width" => 100, "height" => 100})

      expect(png_string).not_to be_empty
    end
  end
end
