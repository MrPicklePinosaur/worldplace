pragma solidity ^0.8.9;

contract Worldplace {

    uint _value;

    constructor(uint value) {
        _value = value;
    }

    function increment() public {
        _value += 1;
    }

    function get_value() public view returns(uint) {
        return _value;
    }

}
