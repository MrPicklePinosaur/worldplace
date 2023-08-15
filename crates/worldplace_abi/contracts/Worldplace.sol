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

  uint32 public width;
  uint32 public height;
  uint32 public cooldown;

  mapping(bytes32 => uint32) internal grid;

  constructor(uint32 _width, uint32 _height, uint32 _cooldown) {
    require(_width > 0);
    require(_height > 0);
    width = _width;
    height = _height;
    cooldown= _cooldown;

  }

  function _bounds_check(Pos memory pos) internal view returns(bool) {
    return (0 <= pos.x && pos.x < width && 0 <= pos.y && pos.y < height);
  }

  // TODO
  /*
  function _color_check(Color color) internal returns(bool) {
    return false;
  }
  */

  modifier canPlace() {
        require(block.timestamp - userCooldowns[msg.sender] >= cooldown, "Cooldown period not passed");
        _;
  }
  function set_pixel(Pos calldata pos, uint8 r, uint8 g, uint8 b, uint8 a) public canPlace{
    require(_bounds_check(pos));
    // TODO validate color too
    userCooldowns[msg.sender] = block.timestamp;
    grid[keccak256(abi.encode(pos))] = encodeColor(r,g,b,a);
  }

  function get_pixel(Pos memory pos) public view returns(uint8, uint8, uint8, uint8) {
    require(_bounds_check(pos));
    uint32 c = grid[keccak256(abi.encode(pos))];
    return decodeColor(c);
  }
  function get_place() external view returns(uint8[][] memory){
    uint8[][] memory array = new uint8[][](height);
    for (uint32 y = 0; y < height; y++) {
      for (uint32 x = 0; x< width; x++) {
        Pos memory pos = Pos(x,y);
        (uint8 r,uint8 g,uint8 b,uint8 a) = get_pixel(pos);
        uint32 p = y*height+x;
        array[p] = new uint8[](4);
        array[p][0] = r;
        array[p][1] = g;
        array[p][2] = b;
        array[p][3] = a;

      }
    }
    return array;


  }

  // TODO
  /* function get_region(Pos top_left, Pos bottom_right) public view returns(Color[]) { */

  /* } */


}
