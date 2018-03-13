package entity

final case class {{ entityName }}Entity (
{%for column in column_list %}  {{column.columnNameCamel}}: {{column.dataType}},
  {% endfor %})
)

object {{ entityName }} {
  implicit def {{ table_name }}Reads: Reads[{{ entityName }}Entity] = (
  {%for column in column_list %}  (JsPath \ "{{column.columnName}}").read[{{column.dataType}}] and
  {% endfor %} )({{ entityName }}Entity.apply _)

  implicit def {{ table_name }}Writes: Writes[{{ entityName }}Entity] = (
  {%for column in column_list %}  (JsPath \ "{{column.columnName}}").write[{{column.dataType}}] and
  {% endfor %} )(unlift({{ entityName }}Entity.unapply))
}

