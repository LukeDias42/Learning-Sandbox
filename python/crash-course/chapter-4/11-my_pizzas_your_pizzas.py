pizzas = ["egg", "bacon", "pepper"]
friend_pizzas = pizzas[:]

pizzas.append("cheese")
print("My favorite pizzas are:")
for pizza in pizzas:
    print(pizza.capitalize())

friend_pizzas.append("shrimp")
print("\nMy friend's favorite pizzas are:")
for pizza in friend_pizzas:
        print(pizza.capitalize())

