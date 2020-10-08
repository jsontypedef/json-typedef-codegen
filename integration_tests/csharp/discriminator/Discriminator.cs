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

                    objectType == typeof(V1)  ||

                    objectType == typeof(V2) 
;
            }

            public override object ReadJson(JsonReader reader, System.Type objectType, object existingValue, JsonSerializer serializer)
            {
                JObject obj = JObject.Load(reader);
                JProperty discriminator = obj.Property("version");

                if (discriminator == null || discriminator.Value.Type != JTokenType.String)
                {
                    throw new JsonSerializationException("bad discriminator property");
                }

                string discriminatorValue = (string) discriminator.Value;
                switch (discriminatorValue)
                {

                    case "v1":
                    {
                        V1 value = new V1();
                        serializer.Populate(obj.CreateReader(), value);
                        return value;
                    }

                    case "v2":
                    {
                        V2 value = new V2();
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
