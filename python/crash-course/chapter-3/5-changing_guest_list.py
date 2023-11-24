guests = ["Jeniffer", "Denise", "Rachel"]
print(f"Hello, {guests[0]}!")
print(f"Hello, {guests[1]}!")
print(f"Hello, {guests[2]}!")

print(f"{guests[1]} won't be able to come...")
guests[1] = "Matheus"

print(f"Hello, {guests[1]}!")