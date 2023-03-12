import HashMap "mo:base/HashMap";
import Array "mo:base/Array";
import List "mo:base/List";
import Buffer "mo:base/Buffer";
import Nat "mo:base/Nat";
import Nat8 "mo:base/Nat8";
import Nat64 "mo:base/Nat64";
import Int "mo:base/Int";
import Int64 "mo:base/Int64";
import Time "mo:base/Time";
import Text "mo:base/Text";
import Hash "mo:base/Hash";
import Bool "mo:base/Bool";
import Principal "mo:base/Principal";
import Cycles "mo:base/ExperimentalCycles";
import Debug "mo:base/Debug";
import Iter "mo:base/Iter";
import Error "mo:base/Error";
import Result "mo:base/Result";

actor {

    public type ArgValue = {
        #I64 : Int64;
        #U64 : Nat64;
        #Vec : [ArgValue];
        #Slice : [Nat8];
        #Text : Text;
        #True;
        #False;
        #Float : Float;
        #Principal : Principal;
    };

    private var data: HashMap.HashMap<Text, [(Text, ArgValue)]> = HashMap.HashMap<Text, [(Text, ArgValue)]>(0, Text.equal, Text.hash);

    public shared func confirm(args: [(Text, ArgValue)]) : async Bool {
        let temp = HashMap.fromIter<Text, ArgValue>(args.vals(), 0, Text.equal, Text.hash);
        let id = switch(temp.get("id")) {
            case (?value) { 
                switch(value) {
                    case (#Text(id)) { id };
                    case (_) { "0" };
                }
            };
            case (null) { "0" };
        };
        if (id != "2") {
            data.put(id, args);
            true
        } else {
            false
        }
    };

    public shared func rollback(args: [(Text, ArgValue)]) : async Bool {
        let temp = HashMap.fromIter<Text, ArgValue>(args.vals(), 0, Text.equal, Text.hash);
        let id = switch(temp.get("id")) {
            case (?value) { 
                switch(value) {
                    case (#Text(id)) { id };
                    case (_) { "0" };
                }
            };
            case (null) { "0" };
        };
        data.delete(id);
        true
    };

    public query func find() : async [(Text, [(Text, ArgValue)])] {
        Iter.toArray(data.entries())
    };

    public query func cycleBalance() : async Nat {
        return Cycles.balance();
    };
    public shared(msg) func cycleAvailable() : async Nat {
        return Cycles.available();
    };

    /// ---------------------
	/// Life cycle functions
	/// ---------------------
	system func preupgrade() {
	};

	system func postupgrade() {
	};
};