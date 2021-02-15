package com.example;

import com.fasterxml.jackson.core.JsonGenerator;
import com.fasterxml.jackson.core.JsonParser;
import com.fasterxml.jackson.core.JsonProcessingException;
import com.fasterxml.jackson.databind.DeserializationContext;
import com.fasterxml.jackson.databind.JsonDeserializer;
import com.fasterxml.jackson.databind.JsonSerializer;
import com.fasterxml.jackson.databind.SerializerProvider;
import com.fasterxml.jackson.databind.annotation.JsonDeserialize;
import com.fasterxml.jackson.databind.annotation.JsonSerialize;

import java.io.IOException;

@JsonSerialize(using = UnsignedShort.Serializer.class)
@JsonDeserialize(using = UnsignedShort.Deserializer.class)
public class UnsignedShort {
    private short value;

    public UnsignedShort(short value) {
        this.value = value;
    }

    public short getValue() {
        return value;
    }

    public void setValue(short value) {
        this.value = value;
    }

    public static class Serializer extends JsonSerializer<UnsignedShort> {
        @Override
        public void serialize(UnsignedShort value, JsonGenerator gen, SerializerProvider serializers) throws IOException {
            gen.writeNumber(Short.toUnsignedLong(value.getValue()));
        }
    }

    public static class Deserializer extends JsonDeserializer<UnsignedShort> {
        @Override
        public UnsignedShort deserialize(JsonParser p, DeserializationContext ctxt) throws IOException, JsonProcessingException {
            return new UnsignedShort((short) p.getLongValue());
        }
    }
}
