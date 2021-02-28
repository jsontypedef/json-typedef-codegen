require 'json'
require_relative 'gen/jtd_codegen_e2e'

$stdout.sync = true
$stdin.each do |line|
  value = JtdCodegenE2e::MAIN.from_json(JSON.parse(line))
  puts JSON.generate(value.to_json)
end
