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


        /// <value>
        /// The first known location of this user
        /// </value>
        [JsonProperty("first_known_location")]
        public Location FirstKnownLocation { get; set; }

        /// <value>
        /// The ID of the user in our database.
        /// </value>
        [JsonProperty("id")]
        public string Id { get; set; }

        /// <value>
        /// Free-form labels that we have put on the user.
        /// </value>
        [JsonProperty("labels")]
        public IDictionary<string, string> Labels { get; set; }

        /// <value>
        /// The last known location of this user
        /// </value>
        [JsonProperty("last_known_location")]
        public Location LastKnownLocation { get; set; }

        /// <value>
        /// The user's name.
        /// </value>
        [JsonProperty("name")]
        public Name Name { get; set; }

        /// <value>
        /// Some preferences the user has indicated to us.
        /// </value>
        [JsonProperty("preferences")]
        public Preferences Preferences { get; set; }





    }
}
