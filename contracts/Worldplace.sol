pragma solidity ^0.8.9;

contract Worldplace {

  struct Pos {
    uint x;
    uint y;
  }

  struct Color {
    uint r;
    uint g;
    uint b;
  }

  uint WIDTH;
  uint HEIGHT;

  mapping(bytes32 => Color) internal grid;

  constructor(uint width, uint height) {
    require(width > 0);
    require(height > 0);
    WIDTH = width;
    HEIGHT = height;
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

  function set_pixel(Pos calldata pos, Color calldata color) public {
    require(_bounds_check(pos));
    // TODO validate color too
    grid[keccak256(abi.encode(pos))] = color;
  }

  function get_pixel(Pos calldata pos) public view returns(uint, uint, uint) {
    require(_bounds_check(pos));
    Color memory c = grid[keccak256(abi.encode(pos))];
    return (c.r, c.g, c.b);
  }

  // TODO
  /* function get_region(Pos top_left, Pos bottom_right) public view returns(Color[]) { */

  /* } */


}
