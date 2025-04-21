# frozen_string_literal: true

if ENV["BUNDLER_ORIG_BUNDLE_GEMFILE"].include?("gemfiles/plain_ruby.gemfile")
  RSpec.configure do |c|
    c.filter_run_excluding type: lambda { |type|
      %i[controller request railtie].include?(type)
    }
  end
  UsersController = Class.new
  return
else
  require "rails"
end

ENV["RAILS_ENV"] ||= "test"

require "bundler/setup"

require_relative "./spec_helper"
require_relative "./dummy/config/application"
require "himg/railtie"

Rails.application.initialize!

require "rspec/rails"
