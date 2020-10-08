using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{
    /// <summary>
    /// User preferences around do-not-track
    /// </summary>
    [JsonConverter(typeof(PreferencesDoNotTrack.JsonConverter))]
    public abstract class PreferencesDoNotTrack 
    {






        private class JsonConverter : Newtonsoft.Json.JsonConverter
        {
            public override bool CanRead => true;
            public override bool CanWrite => false;

            public override bool CanConvert(System.Type objectType)
            {
                return

                    objectType == typeof(PreferencesDoNotTrackV1)  ||

                    objectType == typeof(PreferencesDoNotTrackV0) 
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
                        PreferencesDoNotTrackV1 value = new PreferencesDoNotTrackV1();
                        serializer.Populate(obj.CreateReader(), value);
                        return value;
                    }

                    case "v0":
                    {
                        PreferencesDoNotTrackV0 value = new PreferencesDoNotTrackV0();
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
