appraise "plain-ruby" do
  remove_gem "appraisal"
  remove_gem "steep"
  remove_gem "stackprof"
  remove_gem "rails"
  remove_gem "puma"
end

appraise "rails-6" do
  remove_gem "appraisal"
  remove_gem "steep"
  remove_gem "stackprof"
  remove_gem "puma"
  gem "rails", "~> 6.0"
  gem "rspec-rails", "~> 6.0"
  gem "concurrent-ruby", "1.3.4" # Logger dependency fix, see: https://stackoverflow.com/questions/79360526/uninitialized-constant-activesupportloggerthreadsafelevellogger-nameerror
  gem "bigdecimal", "~> 1.4" # See: https://github.com/rails/rails/issues/34822
  gem "drb"
end

appraise "rails-7-0" do
  remove_gem "appraisal"
  remove_gem "steep"
  remove_gem "stackprof"
  remove_gem "puma"
  gem "rails", "~> 7.0.0"
  gem "rspec-rails", "~> 7.0"
  gem "concurrent-ruby", "1.3.4" # Logger dependency fix, see: https://stackoverflow.com/questions/79360526/uninitialized-constant-activesupportloggerthreadsafelevellogger-nameerror
  gem "bigdecimal", "~> 1.4" # See: https://github.com/rails/rails/issues/34822
  gem "drb"
end

appraise "rails-7-1" do
  remove_gem "appraisal"
  remove_gem "steep"
  remove_gem "stackprof"
  remove_gem "puma"
  gem "rails", "~> 7.1.0"
  gem "rspec-rails", "~> 7.0"
end

appraise "rails-7-2" do
  remove_gem "appraisal"
  remove_gem "steep"
  remove_gem "stackprof"
  remove_gem "puma"
  gem "rails", "~> 7.2.0"
  gem "rspec-rails", "~> 7.0"
end

appraise "rails-8" do
  remove_gem "appraisal"
  remove_gem "steep"
  remove_gem "stackprof"
  remove_gem "puma"
  gem "rails", "~> 8.0"
  gem "rspec-rails", "~> 7.0"
end
