guests = ["Jeniffer", "Denise", "Rachel"]
print(f"Hello, {guests[0]}!")
print(f"Hello, {guests[1]}!")
print(f"Hello, {guests[2]}!")
print("More seats at the table were found!")

guests.insert(0, "Paulinho")
guests.insert(1, "Matheus")
guests.append("Bombadil")

print(f"Hello, {guests[0]}!")
print(f"Hello, {guests[1]}!")
print(f"Hello, {guests[5]}!")

print("Unfortunately the new table won't arrive")
print(f"Sorry you won't be able to come, {guests.pop()}")
print(f"Sorry you won't be able to come, {guests.pop()}")
print(f"Sorry you won't be able to come, {guests.pop()}")
print(f"Sorry you won't be able to come, {guests.pop()}")

print(f"{guests[0]}, you may still come!")
print(f"{guests[1]}, you may still come!")

del guests[1]
del guests[0]
print(guests)