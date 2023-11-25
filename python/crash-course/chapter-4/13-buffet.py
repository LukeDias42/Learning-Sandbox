menu = ['pizza', 'hamburguer', 'pudin', 'spaghetti', 'assassina']

print("Available items:")
for item in menu:
    print(item)

menu = ['onion soup', 'tomato soup', menu[2], menu[3], menu[4]]

print("\nNew items:")
for item in menu:
    print(item)
