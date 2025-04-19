# Himg: The Hyper Image Generator

You give it HTML and it gives back an image!

Parses a minimal subset of HTML/CSS, fetches nested resources, renders an image on the GPU.

Uses rust libraries to do this in a fast, hopefully safe way.

In Rails this will mean you can process user.himg.erb to display an image including data from a user's profile.

## CAVEATS

1. This is **alpha** software, don't expect it to work yet.
2. Rendering requires a GPU. Awaiting CPU support in vello, which Canva may be working on.
3. Performance needs tuning. Both in the underlaying blitz library and how data is passed between Rust and Ruby
4. Network requests can be made: don't use this library with untrusted inputs.
5. file:// URLs are resolved: this could expose files on your computer.

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

### Run from Ruby

```ruby
png = Himg.render("<html bgcolor='blue'></html>") 
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake spec` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

### Run directly from the command line to output an image
```bash
bundle exec cargo run --example file
bundle exec cargo run --example file -- path/to/file.html
```

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/jamedjo/himg.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
