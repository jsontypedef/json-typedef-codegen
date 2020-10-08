using System;
using System.Collections.Generic;
using Newtonsoft.Json;
using Newtonsoft.Json.Linq;

namespace Jtd.JtdCodegenDemo
{
    /// <summary>
    /// A user represents a person in our system.
    /// </summary>
    
    public class User 
    {


        /// <summary>
        /// The first known location of this user
        /// </summary>
        [JsonProperty("first_known_location")]
        public Location FirstKnownLocation { get; set; }

        /// <summary>
        /// The ID of the user in our database.
        /// </summary>
        [JsonProperty("id")]
        public string Id { get; set; }

        /// <summary>
        /// Free-form labels that we have put on the user.
        /// </summary>
        [JsonProperty("labels")]
        public IDictionary<string, string> Labels { get; set; }

        /// <summary>
        /// The last known location of this user
        /// </summary>
        [JsonProperty("last_known_location")]
        public Location LastKnownLocation { get; set; }

        /// <summary>
        /// The user's name.
        /// </summary>
        [JsonProperty("name")]
        public Name Name { get; set; }

        /// <summary>
        /// Some preferences the user has indicated to us.
        /// </summary>
        [JsonProperty("preferences")]
        public Preferences Preferences { get; set; }





    }
}
