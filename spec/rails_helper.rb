# frozen_string_literal: true

if %r{gemfiles/rails.*\.gemfile}.match?(ENV["BUNDLER_ORIG_BUNDLE_GEMFILE"])
  require "rails"
else
  RSpec.configure do |c|
    c.filter_run_excluding type: lambda { |type|
      %i[controller request railtie].include?(type)
    }
  end
  UsersController = Class.new
  return
end

ENV["RAILS_ENV"] ||= "test"

require "bundler/setup"

require_relative "./spec_helper"
require_relative "./dummy/config/application"
require "himg/railtie"

Rails.application.initialize!

require "rspec/rails"
