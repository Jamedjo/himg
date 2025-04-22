# Himg: The Hyper Image Generator

You give it HTML and it gives back an image!

Parses a minimal subset of HTML/CSS, fetches nested resources, renders an image on the GPU.

Uses rust libraries to do this in a fast, hopefully safe way.

In Rails this will mean you can process user.himg.erb to display an image including data from a user's profile.

## CAVEATS

1. This is **pre-alpha** software, don't expect it to work yet.
2. Rendering requires a GPU. Awaiting CPU support in vello, which Canva may be working on.
3. Performance needs tuning. Both in the underlying blitz library and how data is passed between Rust and Ruby
4. Network requests can be made: don't use this library with untrusted inputs.
5. file:// URLs are resolved: this could expose files on your computer.
6. Native extensions are not yet being published for different os/arch

## Installation

Install the gem and add to the application's Gemfile by executing:

```bash
bundle add himg
```

If bundler is not being used to manage dependencies, install the gem by executing:

```bash
gem install himg
```

## Usage

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

### Adding OpenGraph Meta Tags

Once you've added a view template for your resource, you can use it to generate image cards that will be shown when the page is shared on messenger apps or social media.

```html
<meta property="og:title" content="<%= @user.username %>" />
<meta property="og:description" content="<%= @user.tagline %>" />
<meta property="og:image" content="<%= user_url(@user.username, format: :png) %>" />
```

### Configuration Options

Options: `width`, `height`, `verbose`, `truncate`.

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

### How it works

No browser, just basics!

Himg calls through to the amazing blitz library, which uses Stylo to parse the CSS, servo/html5ever to parse the HTML, fetches network resources, builds a scene graph and hands over to vello to render an image.

Interaction between Ruby & Rust is done with the help of `magnus`, `rb_sys` and lots of glue code from the `oxidize-rb` team.

To play nicely with Rails a template handler is registered, which Rails' `default_render` method automatically calls when the corresponding view is found. This can be `show.himg` for a static image, or `show.himg.erb` to use variables from the controller. Additionally a Renderer is available with `render himg: 'content'` in case a view template is not needed.

### Run directly from the command line to output an image

```bash
bundle exec cargo run --example file
bundle exec cargo run --example file -- path/to/file.html
```

## Development

1. Run `bin/setup` to install dependencies.
2. Run `rake spec` to run the tests with the default development setup
3. Run `appraisal rake spec` to run tests against different versions of rails and to confirm that the gem works in a plain ruby environment
4. Run `bin/console` for an interactive prompt that will allow you to experiment.
5. Run `RAILS_ENV=development bundle exec spec/dummy/dummy_rails server` to check the dummy app.
  - http://localhost:3000/users/jamedjo.png will display an opengraph compatible png
  - http://localhost:3000/users/jamedjo.himg will also render the same png
  - http://localhost:3000/users/jamedjo will render an HTML page with opengraph meta tags
6. To install this gem onto your local machine, run `bundle exec rake install`.

## Releases

To release a new version:
1. Run `rake bump:patch` to update the version numbers in `version.rb` and `ext/himg/Cargo.toml`.
3. Run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/jamedjo/himg.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
