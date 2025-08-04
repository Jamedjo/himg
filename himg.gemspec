# frozen_string_literal: true

require_relative "lib/himg/version"

Gem::Specification.new do |spec|
  spec.name = "himg"
  spec.version = Himg::VERSION
  spec.authors = ["James Edwards-Jones"]
  spec.email = ["git@jamedjo.co.uk"]

  spec.summary = "Renders HTML to an Image, perfect for OpenGraph social cards"
  spec.description = "You give it HTML and it renders an Image! No browser required. Perfect for OpenGraph social cards so your links don't look boring when shared."

  spec.homepage = "https://github.com/Jamedjo/Himg"
  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.1.0"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/Jamedjo/Himg"
  spec.metadata["changelog_uri"] = "https://github.com/Jamedjo/Himg/blob/CHANGELOG.md"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  gemspec = File.basename(__FILE__)
  spec.files = IO.popen(%w[git ls-files -z], chdir: __dir__, err: IO::NULL) do |ls|
    ls.readlines("\x0", chomp: true).reject do |f|
      (f == gemspec) ||
        f.start_with?(*%w[bin/ test/ spec/ features/ .git .github appveyor Gemfile])
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/himg/extconf.rb"]

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"
  spec.add_dependency "rb_sys", "~> 0.9"

  spec.add_development_dependency "bump"
  spec.add_development_dependency "debug"
  spec.add_development_dependency "irb"
  spec.add_development_dependency "rake"
  spec.add_development_dependency "rake-compiler"
  spec.add_development_dependency "rspec"
  spec.add_development_dependency "rubocop"
  spec.add_development_dependency "thor"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
