# frozen_string_literal: true

require "spec_helper"

RSpec.describe Himg do
  def dimensions(png_data)
    png_data[0x10..0x18].unpack('NN')
  end

  def width(png_data)
    dimensions(png_data).first
  end

  def height(png_data)
    dimensions(png_data).last
  end

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

  it "defaults to good dimensions for opengraph images" do
    png_string = Himg.render("<html></html>")

    expect(dimensions(png_string)).to eq [720, 405]
  end

  it "allows width to be configured" do
    png_string = Himg.render("<html></html>", width: 100)

    expect(width(png_string)).to eq(100)
  end

  it "allows height to be configured" do
    png_string = Himg.render("<div>Tall</div>", height: 5)

    expect(height(png_string)).to eq(5)
  end

  it "allows string values for height and width" do
    png_string = Himg.render("<html></html>", width: "999", height: "99")

    expect(dimensions(png_string)).to eq [999, 99]
  end

  it "allows height truncation to be turned off" do
    png_string = Himg.render("<div>Tall</div>", height: 5, truncate: false)

    expect(height(png_string)).to be > 5
  end

  it "does not log to stdout by default" do
    expect { Himg.render("<!DOCTYPE html>") }.not_to output.to_stdout_from_any_process
  end

  it "allows verbose logging to be turned on" do
    expect { Himg.render("<!DOCTYPE html>", verbose: true) }.to output(
      a_string_matching(/Screenshot is \(720x405\)/) &
      a_string_matching(/Rendered to buffer in \d+ms/)
    ).to_stdout_from_any_process
  end

  it "fetches resources relative to a base_url" do
    fixture_path = Pathname.new(__FILE__).parent.join("fixtures")
    html_with_embed = "<!DOCTYPE html><img src='./relative.svg'/>"

    expect { Himg.render(html_with_embed, base_url: fixture_path, verbose: true) }.to output(
      a_string_matching(/Success file.*fixtures\/relative\.svg/)
    ).to_stdout_from_any_process
  end
end
