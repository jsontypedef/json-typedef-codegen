# Code generated by jtd-codegen for Ruby v0.2.0

module JtdCodegenE2e

  class Root
    attr_accessor :value

    def initialize(value)
      self.value = value
    end

    private_class_method :new

    FOO = new("FOO")
    FOO0 = new("Foo")
    FOO1 = new("foo")

    def self.from_json(data)
      {
        "FOO" => FOO,
        "Foo" => FOO0,
        "foo" => FOO1,
      }[data]
    end

    def to_json
      value
    end
  end

  private

  def self.from_json(type, data)
    if data.nil? || [Object, TrueClass, Integer, Float, String].include?(type)
      data
    elsif type.is_a?(Array)
      data.map { |elem| from_json(type.first, elem) }
    elsif type.is_a?(Hash)
      data.transform_values { |elem| from_json(type.values.first, elem) }
    else
      type.from_json(data)
    end
  end

  def self.to_json(data)
    if data.nil? || [TrueClass, FalseClass, Integer, Float, String].include?(data.class)
      data
    elsif data.is_a?(Array)
      data.map { |elem| to_json(elem) }
    elsif data.is_a?(Hash)
      data.transform_values { |elem| to_json(elem) }
    else
      data.to_json
    end
  end
end
