module default {
    type Todo {
        required property todo_id -> int64 {
            readonly := true;
            constraint exclusive;
        };
        required property title -> str;
        required property completed -> bool {
            default := false;
        };
        required property created_at -> datetime {
            default := datetime_current();
        };
        property description -> str;

        index on (.todo_id);
    };

    type User {
        required property name -> str {
            constraint min_len_value(3);
            constraint max_len_value(15);
            constraint regexp("[a-zA-Z0-9_-]+");
        };
        required property user_id -> int64 {
            constraint exclusive;
        };
        required property password -> str;

        multi link todo -> Todo {
            on target delete allow;
        };
    }
}
