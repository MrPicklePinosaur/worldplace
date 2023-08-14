pragma solidity ^0.8.9;

contract Worldplace {

  struct Pos {
    uint32 x;
    uint32 y;
  }


  function encodeColor(uint8 r, uint8 g, uint8 b, uint8 a) public pure returns (uint32) {
        uint32 color = (uint32(r) << 24) | (uint32(g) << 16) | (uint32(b) << 8) | uint32(a);
        return color;

  }
  function decodeColor(uint32 color) public pure returns (uint8, uint8, uint8, uint8) {
        uint8 r = uint8((color >> 24) & 0xFF);
        uint8 g = uint8((color >> 16) & 0xFF);
        uint8 b = uint8((color >> 8) & 0xFF);
        uint8 a = uint8(color & 0xFF);
        return (r, g, b, a);
  }
  mapping(address => uint256) public userCooldowns; 

  uint WIDTH;
  uint HEIGHT;
  uint COOLDOWN;

  mapping(bytes32 => uint32) internal grid;

  constructor(uint width, uint height, uint cooldown) {
    require(width > 0);
    require(height > 0);
    WIDTH = width;
    HEIGHT = height;
    COOLDOWN = cooldown;
  }

  function _bounds_check(Pos memory pos) internal view returns(bool) {
    return (0 <= pos.x && pos.x < WIDTH && 0 <= pos.y && pos.y < HEIGHT);
  }

  // TODO
  /*
  function _color_check(Color color) internal returns(bool) {
    return false;
  }
  */

  modifier canPlace() {
        require(block.timestamp - userCooldowns[msg.sender] >= COOLDOWN, "Cooldown period not passed");
        _;
  }
  function set_pixel(Pos calldata pos, uint8 r, uint8 g, uint8 b, uint8 a) public canPlace{
    require(_bounds_check(pos));
    // TODO validate color too
    grid[keccak256(abi.encode(pos))] = encodeColor(r,g,b,a);
  }

  function get_pixel(Pos calldata pos) public view returns(uint8, uint8, uint8, uint8) {
    require(_bounds_check(pos));
    uint32 c = grid[keccak256(abi.encode(pos))];
    return decodeColor(c);
  }

  // TODO
  /* function get_region(Pos top_left, Pos bottom_right) public view returns(Color[]) { */

  /* } */


}
