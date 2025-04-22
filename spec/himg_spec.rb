# frozen_string_literal: true

RSpec.describe Himg do
  it "has a version number" do
    expect(Himg::VERSION).not_to be nil
  end

  it "converts HTML to an Image" do
    png_string = Himg.render("<html></html>")

    expect(png_string).to start_with("\x89PNG\r\n\x1A\n".b)
  end

  it "finishes writing the png" do
    png_string = Himg.render("<html></html>")

    expect(png_string).to end_with("IEND\xAEB`\x82".b)
  end
end
