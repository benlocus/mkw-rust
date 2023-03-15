BEGIN TRANSACTION;

# TABLES

// users table
DEFINE TABLE users SCHEMAFULL;

DEFINE FIELD profileid ON TABLE users TYPE int; // should be an int
DEFINE FIELD userid ON TABLE users TYPE string;
DEFINE FIELD password ON TABLE users TYPE string;
DEFINE FIELD gsbrcd ON TABLE users TYPE string;
DEFINE FIELD email ON TABLE users TYPE string
    ASSERT is::email($value);
DEFINE FIELD uniquenick ON TABLE users TYPE string;
DEFINE FIELD pid ON TABLE users TYPE string;
DEFINE FIELD lon ON TABLE users TYPE float;
DEFINE FIELD lat ON TABLE users TYPE float;
DEFINE FIELD loc ON TABLE users TYPE string;
DEFINE FIELD firstname ON TABLE users TYPE string;
DEFINE FIELD lastname ON TABLE users TYPE string;
DEFINE FIELD stat ON TABLE users TYPE string;
DEFINE FIELD partnerid ON TABLE users TYPE record (users); // probably should be a record
DEFINE FIELD console ON TABLE users TYPE string;
DEFINE FIELD csnum ON TABLE users TYPE string;
DEFINE FIELD cfc ON TABLE users TYPE string;
DEFINE FIELD bssid ON TABLE users TYPE string;
DEFINE FIELD devname ON TABLE users TYPE string; // may need to be a blob?
DEFINE FIELD birth ON TABLE users TYPE datetime; // prob needs to be a datetime
DEFINE FIELD gameid ON TABLE users TYPE string; // prob should be a record
DEFINE FIELD enabled ON TABLE users TYPE bool; // probably a boolean (is an int in the original)
DEFINE FIELD zipcode ON TABLE users TYPE string
    ASSERT is::numeric($value) AND string::length($value) = 5; // zip code validation?
DEFINE FIELD aim ON TABLE users TYPE string;

// session table
DEFINE TABLE sessions SCHEMAFULL;

DEFINE FIELD session ON TABLE sessions TYPE string; // pretty sure this should be an int but idk what it's used for
DEFINE FIELD profileid ON TABLE sessions TYPE record (users); // should be a record of user
DEFINE FIELD loginticket ON TABLE sessions TYPE string; // should be an int

// buddies table
DEFINE TABLE buddies SCHEMAFULL;

DEFINE FIELD userProfileId ON TABLE buddies TYPE record (users); // should be an int
DEFINE FIELD buddyProfileId ON TABLE buddies TYPE record (users); // should be an int
DEFINE FIELD time ON TABLE buddies TYPE datetime;
DEFINE FIELD status ON TABLE buddies TYPE int; // might not be an int
DEFINE FIELD notified ON TABLE buddies TYPE int; // should maybe be a bool
DEFINE FIELD gameid ON TABLE buddies TYPE string; // should be an int
DEFINE FIELD blocked ON TABLE buddies TYPE int; // should maybe be a bool

// pending_messages table
DEFINE TABLE pending_messages SCHEMAFULL;

DEFINE FIELD sourceid ON TABLE pending_messages TYPE record (users);
DEFINE FIELD targetid ON TABLE pending_messages TYPE record (users);
DEFINE FIELD msg ON TABLE pending_messages TYPE string;

// gamestat_profile table
DEFINE TABLE gamestat_profile SCHEMAFULL;

DEFINE FIELD profileid ON TABLE gamestat_profile TYPE record (users);
DEFINE FIELD dindex ON TABLE gamestat_profile TYPE string; // maybe an int?
DEFINE FIELD ptype ON TABLE gamestat_profile TYPE string;

// gameinfo table
DEFINE TABLE gameinfo SCHEMAFULL;

DEFINE FIELD profileid ON TABLE gameinfo TYPE record (users);
DEFINE FIELD dindex ON TABLE gameinfo TYPE string; // maybe an int?
DEFINE FIELD ptype ON TABLE gameinfo TYPE string;
DEFINE FIELD data ON TABLE gameinfo TYPE string;

// nas_logins table
DEFINE TABLE nas_logins SCHEMAFULL;

DEFINE FIELD userid ON TABLE nas_logins TYPE string;
DEFINE FIELD authtoken ON TABLE nas_logins TYPE string;
DEFINE FIELD data ON TABLE nas_logins TYPE string;

// banned table
DEFINE TABLE banned SCHEMAFULL;

DEFINE FIELD gameid ON TABLE banned TYPE string;
DEFINE FIELD ipaddr ON TABLE banned TYPE string;

// pending table
DEFINE TABLE pending SCHEMAFULL;

DEFINE FIELD macadr ON TABLE pending TYPE string;

// registered table
DEFINE TABLE registered SCHEMAFULL;

DEFINE FIELD macadr ON TABLE registered TYPE string;


# INDEXES
DEFINE INDEX gamestatprofile_triple ON TABLE gamestat_profile COLUMNS profileid, dindex, ptype UNIQUE;

DEFINE INDEX users_profileid_idx ON TABLE users COLUMNS profileid UNIQUE;
DEFINE INDEX users_userid_idx ON TABLE users COLUMNS userid;

DEFINE INDEX pending_messages_targetid_idx ON TABLE pending_messages COLUMNS targetid;

DEFINE INDEX sessions_session_idx ON TABLE sessions COLUMNS session UNIQUE;
DEFINE INDEX sessions_loginticket_idx ON TABLE sessions COLUMNS loginticket;
DEFINE INDEX sessions_profileid_idx ON TABLE sessions COLUMNS profileid;

DEFINE INDEX nas_logins_authtoken_idx ON TABLE nas_logins COLUMNS authtoken UNIQUE;

DEFINE INDEX nas_logins_userid_idx ON TABLE nas_logins COLUMNS userid;

DEFINE INDEX buddies_userProfileId_idx ON TABLE buddies COLUMNS userProfileId;
DEFINE INDEX buddies_buddyProfileId_idx ON TABLE buddies COLUMNS buddyProfileId;

DEFINE INDEX buddies_buddyProfileId_idx ON TABLE buddies COLUMNS buddyProfileId;

DEFINE INDEX gamestat_profile_profileid_idx ON TABLE gamestat_profile COLUMNS profileid;

COMMIT TRANSACTION;
