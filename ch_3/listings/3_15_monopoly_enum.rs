enum Turn {
  PassGo,                               //#A
  Collect {money: i32},                 //#B
  Chance,
  CommunityChest,
  GoToJail,
  Purchase {property: Property},        //#C
  FreeParking(i32),                     //#D
}
// #A Enum Variant
// #B Variant with named associated data
// #C Variant with associated custom type
// #D Variant with unnamed associated data
