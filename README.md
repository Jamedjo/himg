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
7. Verbose logging is hardcoded

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
png = Himg.render("<html bgcolor='blue'></html>") 
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

### Advanced Rails Usage

A :himg template handler is registered and will be called by rails' `default_render` method automatically when the corresponding view is found. This can be `show.himg` for a static image, or `show.himg.erb` to use variables from the controller.

If you prefer you could also use `render himg: "<div>My Data</div>"` instead, but should be careful with untrusted input if constructing HTML manually.

To be explicit in the controler you can also use `respond_to` style:

```ruby
respond_to do |format|
  format.html
  format.himg
end
```

```ruby
respond_to do |format|
  format.html
  format.himg { render himg: '<h1 style="text-align: center;">Recent Users</h1>' }
  format.png { render himg: '<div>For .png URLs</div>' }
end
```

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
1. Update the version number in `version.rb`
2. Update the version number in `ext/himg/Cargo.toml`
3. Run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/jamedjo/himg.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
