using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{

    [JsonConverter(typeof(Discriminator.JsonConverter))]
    public abstract class Discriminator 
    {






        private class JsonConverter : Newtonsoft.Json.JsonConverter
        {
            public override bool CanRead => true;
            public override bool CanWrite => false;

            public override bool CanConvert(System.Type objectType)
            {
                return

                    objectType == typeof(DiscriminatorBaz)  ||

                    objectType == typeof(DiscriminatorBar) 
;
            }

            public override object ReadJson(JsonReader reader, System.Type objectType, object existingValue, JsonSerializer serializer)
            {
                JObject obj = JObject.Load(reader);
                JProperty discriminator = obj.Property("foo");

                if (discriminator == null || discriminator.Value.Type != JTokenType.String)
                {
                    throw new JsonSerializationException("bad discriminator property");
                }

                string discriminatorValue = (string) discriminator.Value;
                switch (discriminatorValue)
                {

                    case "baz":
                    {
                        DiscriminatorBaz value = new DiscriminatorBaz();
                        serializer.Populate(obj.CreateReader(), value);
                        return value;
                    }

                    case "bar":
                    {
                        DiscriminatorBar value = new DiscriminatorBar();
                        serializer.Populate(obj.CreateReader(), value);
                        return value;
                    }

                    default:
                        throw new JsonSerializationException("bad discriminator value");
                }
            }

            public override void WriteJson(JsonWriter writer, object value, JsonSerializer serializer)
            {
                throw new NotImplementedException();
            }
        }

    }
}
