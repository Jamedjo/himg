<div align="center">
  <img src="logo.svg" alt="Himg" width="200">
  <div>
    <img src="tagline.svg" alt="The Hyper Image Generator" width="600">
  </div>

  <p>You give it HTML and it gives back an image!</p>

  <p>Parses HTML/CSS, fetches nested resources, renders an image. No browser, no fuss.</p>

  <p>Perfect for OpenGraph images - stop losing clicks to boring links by adding a rich image preview that showcases your content.</p>
</div>

![Mockup showing HTML being transformed into a WhatsApp preview](/readme_hero.svg)

## Quickstart

### Command Line

```bash
gem install himg

himg screenshot path/to/your.html screenshot.png
himg screenshot https://himg.jamedjo.co.uk himg.png --width=1024 --verbose --no-truncate
echo '<h1>Hello Image</h1>' | himg screenshot --stdin output.png
```

### Ruby

```ruby
png = Himg.render("<!DOCTYPE html><body style='background:blue;'></body>")
File.open("blue.png", "wb"){|f| f.write(png) }
```

```ruby
png = Himg.render("<!DOCTYPE html><h1>Snapshot</h1>", width: 89, height: 5, truncate: false)
File.open("dynamic_height.png", "wb"){|f| f.write(png) }
```

### Rails

Simply add a `show.himg.erb`!

```erb
<div><%= @username %></div>
```

### OpenGraph Meta Tags

OpenGraph tags let messenger apps and social media sites know to use your generated image as a thumbnail card for your website.

```html
<meta property="og:title" content="<%= @user.username %>" />
<meta property="og:description" content="<%= @user.tagline %>" />
<meta property="og:image" content="<%= user_url(@user.username, format: :png) %>" />
```

![WhatsApp preview comparison showing plain link vs rich OpenGraph preview](/readme_opengraph_comparison.png)

# Usage

Install the gem and add to the application's Gemfile by executing:

```bash
bundle add himg
```

## Configuration

|Option | Description | Type | Default |
|-|-|-|-|
|width | Sets the width of the rendered content. | integer | 720 |
|height | Sets the desired height of the rendered output. | integer | 405 |
|truncate | Keeps the image height fixed instead of expanding to include the full page | bool | true |
|verbose | Enables detailed logging for debugging and profiling. | bool | false |
|base_url | Where relative paths are relative to for linked resources (stylesheets, images, fonts, etc) | string | nil |
|disable_fetch | Disables fetching linked resources from disk and network| bool | false |
|fetch_timeout | Timeout in seconds for fetching resources | float | 10.0 |
|gpu | Use GPU renderer instead of CPU renderer | bool | false |
|http_headers | Headers sent when the CLI fetches the SOURCE HTML (CLI only) | hash | nil |
|stdin | Read HTML content from stdin instead of a file (CLI only) | bool | false |


### Passing options to a Rails view template

Options can be set at a controller level using the `himg_config` helper method:
```ruby
class UsersController < ActionController::Base
  himg_config(verbose: true)
end
```

These can be overridden at a view level:
```ruby
class UsersController < ActionController::Base
  def show
    himg_config(width: params[:w]) if params[:w]

    @user = User.new
  end
```

### Rails manual render

If you prefer you could also use `render himg: "<div>My Data</div>"` instead, but should be careful with untrusted input if constructing HTML manually.

Options can then be passed directly to the manual render:
```ruby
render himg: '<!DOCTYPE html>', truncate: false
```

Alternatively you can pass in options which have been set with `himg_config`:
```ruby
render himg: '<!DOCTYPE html>', config: himg_config
```

### Rails `respond_to`

To be explicit in the controller you can also use `respond_to` style:

```ruby
respond_to do |format|
  format.html
  format.himg
end
```

You can also use this combined with a manual render:
```ruby
respond_to do |format|
  format.html
  format.himg { render himg: '<h1 style="text-align: center;">Recent Users</h1>' }
  format.png { render himg: '<div>For .png URLs</div>' }
end
```

# Supported HTML and CSS

Himg supports a large subset of the HTML and CSS you'd need to get by, but not all elements or properties are supported.

For a full list, see our [snapshot of the blitz.is status page](https://github.com/Jamedjo/himg/issues/2).

If something you'd like supported is missing and is available [upstream on blitz](https://blitz.is/status), please [open an issue](https://github.com/Jamedjo/himg/issues/new).

|HTML|Status|
|-|-|
|`Generic HTML5 elements`|✅ Supported|
|`<style>`|✅ Supported|
|`<link rel="stylesheet">`|✅ Supported|
|`<link rel="icon">`|❌ Not supported|
|`<img src="">`|✅ Supported|
|`<img srcset="">`|❌ Not supported|
|`<picture>`|❌ Not supported|
|`<svg>`|⚠️ [Partial support](https://github.com/Jamedjo/himg/issues/2)|
|`<h1>-<h6>`|✅ Supported|
|`<ul>/<ol>`|✅ Supported|
|`<i>/<em>`|✅ Supported|
|`<b>/<strong>`|✅ Supported|
|`<u>`|✅ Supported|
|`<center>`|✅ Supported|
|`<pre>/<blockquote>`|✅ Supported|
|`<a>`|✅ Supported|
|`<br>`|✅ Supported|
|`<hr>`|❌ Not supported|
|`<details>/<summary>`|❌ Not supported|
|`<table>`|⚠️ [Partial support](https://github.com/Jamedjo/himg/issues/2)|
|`Form Controls`|⚠️ [Partial support](https://github.com/Jamedjo/himg/issues/2)|

|CSS|Status|
|-|-|
|`display:inline, display:block, display:inline-block`|✅ Supported|
|`display:none`|✅ Supported|
|`display:flex`|✅ Supported|
|`display:grid`|⚠️ [Partial support](https://github.com/Jamedjo/himg/issues/2)|
|`display:table`|⚠️ [Partial support](https://github.com/Jamedjo/himg/issues/2)|
|`display:contents`|❌ Not supported|
|`position:relative, position:absolute`|✅ Supported|
|`position:static, position:fixed, position:sticky`|❌ Not supported|
|`overflow`|⚠️ [Partial support](https://github.com/Jamedjo/himg/issues/2)|
|`z-index`|⚠️ [Partial support](https://github.com/Jamedjo/himg/issues/2)|
|`box-sizing`|✅ Supported|
|`float`|❌ Not supported|
|`content (::before / ::after)`|⚠️ [Partial support](https://github.com/Jamedjo/himg/issues/2)|
|`opacity,  visibility`|✅ Supported|
|`width, height`|✅ Supported|
|`min-width, max-width, min-height, max-height`|✅ Supported|
|`padding, margin, gap`|✅ Supported|
|`border`|⚠️ [Partial support](https://github.com/Jamedjo/himg/issues/2)|
|`@font-face, font-size, font-family`|✅ Supported|
|`font-weight, font-style, font-stretch`|✅ Supported|
|`font-display, font-variant, font-feature-settings`|❌ Not supported|
|`color, text-align, line-height, text-decoration`|✅ Supported|
|`letter-spacing, overlap-wrap, word-wrap, word-break`|✅ Supported|
|`vertical-align, text-transform, text-overflow`|❌ Not supported|
|`border`|⚠️ [Partial support](https://github.com/Jamedjo/himg/issues/2)|
|`background-color, background-image`|✅ Supported|
|`background-size, background-position`|✅ Supported|
|`background-repeat, background-clip, background-origin`|✅ Supported|
|`background-attachment`|❌ Not supported|
|`box-shadow`|✅ Supported|
|`filter`|❌ Not supported|

# How it works

No browser, just basics!

Himg calls through to the amazing blitz library, which uses Stylo to parse the CSS, servo/html5ever to parse the HTML, fetches network resources, builds a scene graph and hands over to vello to render an image.

Interaction between Ruby & Rust is done with the help of `magnus`, `rb_sys` and lots of glue code from the `oxidize-rb` team.

To play nicely with Rails a template handler is registered, which Rails' `default_render` method automatically calls when the corresponding view is found. This can be `show.himg` for a static image, or `show.himg.erb` to use variables from the controller. Additionally a Renderer is available with `render himg: 'content'` in case a view template is not needed.

## CAVEATS

1. This is **pre-alpha** software, don't expect it to work perfectly yet.
2. Performance needs tuning. Both in the underlying blitz library and how data is passed between Rust and Ruby
3. Network requests can be made: don't use this library with untrusted inputs. Use `disable_fetch` if you don't need to fetch any resources.
4. file:// URLs are resolved: this could expose files on your computer. Use `disable_fetch` if you don't need to fetch any resources.

# Development

1. Run `bin/setup` to install dependencies.
2. Run `rake spec` to run the tests with the default development setup
3. Run `appraisal rake spec` to run tests against different versions of rails and to confirm that the gem works in a plain ruby environment
4. Run `bin/console` for an interactive prompt that will allow you to experiment.
5. Run `RAILS_ENV=development bundle exec spec/dummy/dummy_rails server` to check the dummy app.
  - http://localhost:3000/users/jamedjo.png will display an opengraph compatible png
  - http://localhost:3000/users/jamedjo.himg will also render the same png
  - http://localhost:3000/users/jamedjo will render an HTML page with opengraph meta tags
6. To install this gem onto your local machine, run `bundle exec rake install`.
7. To simulate a headless server environment without a GPU, use `WGPU_BACKEND=empty bundle exec rspec`
8. To profile performance with flamegraphs, run `bin/profile spec/fixtures/profile_test.html`

### Run cargo example directly generate image in Rust

```bash
bundle exec cargo run --example file
bundle exec cargo run --example file -- path/to/file.html
```
## Releases

To release a new version:
1. Run `rake bump:patch` to update the version numbers in `version.rb` and `ext/himg/Cargo.toml`.
3. Run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/jamedjo/himg.

## License

Copyright (c) 2025 James Edwards-Jones

This project is dual licenced under both the MIT and Apache 2.0 terms.

See: [MIT License](https://opensource.org/licenses/MIT) and [APACHE 2.0 License](http://www.apache.org/licenses/LICENSE-2.0)
