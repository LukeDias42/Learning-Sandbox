languages =    ["Portuguese Brazil", 
                "Portuguese", 
                "English (USA)", 
                "English (United Kingdom)",
                "Chinese",
                "Chinese (Simplified)",
                "Japanese",
                "French",
                "Russian",
                "Finnish",
                "German",
                "Spanish",
                "Italian"]

languages.append("Korean")
languages.insert(2, "Esperanto")
languages.remove("English (USA)")
del languages[5]

print(languages)
print(languages.pop())

print(sorted(languages))
print(languages)

print(reversed(languages))
print(languages)

languages.sort()
print(languages)

languages.sort(reverse=True)
print(languages)