# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rubocop/rake_task"

RuboCop::RakeTask.new

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("himg.gemspec")

RbSys::ExtensionTask.new("himg", GEMSPEC) do |ext|
  ext.lib_dir = "lib/himg"
end

require "bump/tasks"
Bump.replace_in_default = %w[Cargo.lock ext/himg/Cargo.toml]
Bump.changelog = :editor

task default: %i[compile rubocop]
