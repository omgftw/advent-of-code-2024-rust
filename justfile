# add_day day_number:
#     @cp -r src/template src/day{{day_number}}
#     @sed -i '' "s/{{{{day_number}}/{{day_number}}/g" src/day{{day_number}}/*.tmpl
#     @for file in src/day{{day_number}}/*.tmpl; do mv "$file" "${file%.tmpl}"; done

add_day day_number:
    templative template --day_number {{day_number}}