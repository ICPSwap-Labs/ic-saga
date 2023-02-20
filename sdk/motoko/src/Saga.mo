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

module {

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

    public type EventNodeRequest = {
        name : Text;
        args : [(Text, ArgValue)];
    };

    public type EventRequest = {
        name : Text;
        nodes : [EventNodeRequest];
    };

    type Service = actor {
        execute : shared (EventRequest) -> async Bool;
    };

    public class Saga(
        sagaCid : Text,
    ) {

        let saga: Service = actor(sagaCid);

        public func execute(
            request: EventRequest,
        ) : async Bool {
            await saga.execute(request)
        };
    };

}