from thunder_book.ch05.maze_state import Character


def test_character_eq():
    character = Character(y=1, x=2, mark="A")
    assert character == (1, 2)
    assert character != (1, 3)
    assert character != (2, 2)
