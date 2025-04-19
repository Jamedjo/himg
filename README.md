# Himg

Library for generating an image from HTML

Parses a minimal subset of HTML/CSS, fetches nested resources, renders an image on the GPU.

Uses rust libraries to do this in a fast, hopefully safe way.

In Rails this will mean you can process user.himg.erb to display an image including data from a user's profile.

## Installation

TODO: Replace `UPDATE_WITH_YOUR_GEM_NAME_IMMEDIATELY_AFTER_RELEASE_TO_RUBYGEMS_ORG` with your gem name right after releasing it to RubyGems.org. Please do not do it earlier due to security reasons. Alternatively, replace this section with instructions to install your gem from git if you don't plan to release to RubyGems.org.

Install the gem and add to the application's Gemfile by executing:

```bash
bundle add UPDATE_WITH_YOUR_GEM_NAME_IMMEDIATELY_AFTER_RELEASE_TO_RUBYGEMS_ORG
```

If bundler is not being used to manage dependencies, install the gem by executing:

```bash
gem install UPDATE_WITH_YOUR_GEM_NAME_IMMEDIATELY_AFTER_RELEASE_TO_RUBYGEMS_ORG
```

## Usage

### Run from Ruby

```ruby
png = Himg.render("<html bgcolor='blue'></html>") 
```

### Run directly from the command line to output an image
```bash
bundle exec cargo run --example file
bundle exec cargo run --example file -- path/to/file.html
```

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake spec` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/jamedjo/himg.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
