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

  it "fetches external resources from file:// URLs" do
    fixture_path = Pathname.new(__FILE__).parent.join("fixtures")
    html_with_file_resource = "<!DOCTYPE html><img src='file://#{fixture_path}/absolute.svg'/>"

    expect { Himg.render(html_with_file_resource, verbose: true) }.to output(
      a_string_matching(/Fetched.*absolute/)
    ).to_stdout_from_any_process
  end

  it "fetches external resources from https:// URLs" do
    https_image_link = "https://github.com/github.png?size=5"
    html_with_https_resource = "<!DOCTYPE html><img src='#{https_image_link}'/>"

    expect { Himg.render(html_with_https_resource, verbose: true) }.to output(
      a_string_matching(/Fetched.*github\.png/)
    ).to_stdout_from_any_process
  end

  it "skips fetching external resources with disable_fetch" do
    fixture_path = Pathname.new(__FILE__).parent.join("fixtures")
    html_with_external_resource = "<!DOCTYPE html><img src='file://#{fixture_path}/absolute.svg'/>"

    expect { Himg.render(html_with_external_resource, verbose: true, disable_fetch: true) }.not_to output(
      a_string_matching(/Fetched.*absolute/)
    ).to_stdout_from_any_process
  end

  it "fetching missing resources is handled gracefully" do
    html_with_missing_resource = "<!DOCTYPE html><img src='file:///does_not_exist.png'/>"

    png_string = Himg.render(html_with_missing_resource)

    expect(png_string).not_to be_empty
  end

  it "fetches resources relative to a base_url" do
    fixture_path = Pathname.new(__FILE__).parent.join("fixtures")
    html_with_embed = "<!DOCTYPE html><img src='./relative.svg'/>"

    expect { Himg.render(html_with_embed, base_url: fixture_path, verbose: true) }.to output(
      a_string_matching(/Fetched.*fixtures\/relative\.svg/)
    ).to_stdout_from_any_process
  end

  it "raises a friendly error when attempting to fetch a relative resource but base_url is not set" do
    fixture_path = Pathname.new(__FILE__).parent.join("fixtures")
    html_with_embed = "<!DOCTYPE html><img src='./readme_hero.svg'/>"

    expect { Himg.render(html_with_embed) }.to raise_error(Himg::Error, /to be able to resolve.*base_url/)
  end

  it "raises a friendly error when attempting to fetch a relative resource but base_url is blank" do
    fixture_path = Pathname.new(__FILE__).parent.join("fixtures")
    html_with_embed = "<!DOCTYPE html><img src='./readme_hero.svg'/>"

    expect { Himg.render(html_with_embed, base_url: " ") }.to raise_error(Himg::Error, /to be able to resolve.*base_url/)
  end

  it "limits fetch duration using fetch_timeout" do
    non_resolvable_source = "http://192.0.0.1/missing.svg"
    html_with_embed = "<!DOCTYPE html><img src='#{non_resolvable_source}'/>"

    expect { Himg.render(html_with_embed, verbose: true, fetch_timeout: 0.00001) }.to output(
      a_string_matching(/Timeout fetching assets/)
    ).to_stdout_from_any_process
  end

  it "accepts gpu option for GPU rendering" do
    if ENV['CI']
      expect { Himg.render("<html></html>", gpu: true) }.to raise_error(Himg::GpuNotFound, /No compatible device found/)
    else
      png_string = Himg.render("<html></html>", gpu: true)
      expect(png_string).to start_with("\x89PNG\r\n\x1A\n".b)
    end
  end

  it "handles invalid parameters at the Rust boundary" do
    expect { Himg.render_to_string(123, {}) }.to raise_error(TypeError)
  end

  it "handles malformed URLs that might cause parsing issues" do
    html_with_malformed_urls = <<~HTML
      <!DOCTYPE html>
      <html>
        <head>
          <link rel="stylesheet" href="https://xn--🦀⚡🌐📸🖼️.com/invalid_domain_name.css">
          <link rel="stylesheet" href="http://:80/empty_host.css">
          <link rel="stylesheet" href="http://example.com:badport/bad_port.css">
          <link rel="stylesheet" href="http://example.com:999999/bad_port_number.css">
          <link rel="stylesheet" href="http://256.256.256.256/invalid_ip_address.css">
          <link rel="stylesheet" href="http://[:::1]/invalid_ipv6_address.css">
          <link rel="stylesheet" href="http://exa mple.com/invalid_domain_character.css">
          <link rel="stylesheet" href="./no/base/relative_url_without_base.css">
          <link rel="stylesheet" href="mailto:relative.url@cannot.be.a.base.com">
          <link rel="stylesheet" href="https://example.com:9999999999999999999999/overflow.css">
        </head>
        <body>
          <a href="http://[invalid-ipv6">Invalid IPv6</a>
          <a href="http://example.com:99999">Invalid port</a>
          <a href="http://:80">Missing host</a>
          <a href="http://user@:80">User info without host</a>
          <a href="http://256.256.256.256">Invalid IP</a>
          <img src="http://example.com/path with spaces/image.png">
          <img src="http://example.com/path%ZZ">Invalid percent encoding</img>
        </body>
      </html>
    HTML

    expect { Himg.render(html_with_malformed_urls) }.to raise_error(Himg::Error, /Panic:.*to be able to resolve/)
  end
end
