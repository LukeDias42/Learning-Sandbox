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