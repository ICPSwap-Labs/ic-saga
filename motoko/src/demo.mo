import HashMap "mo:base/HashMap";
import Array "mo:base/Array";
import List "mo:base/List";
import Buffer "mo:base/Buffer";
import Nat "mo:base/Nat";
import Int "mo:base/Int";
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


    public query func test(args: [(Text, ArgValue)]) : async Bool {
        true
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